use std::io;

use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) enum State {
    SynRcvd,
    Estab,
    FinWait1,
    FinWait2,
    Closing,
}

impl State {
    fn is_syn(&self) -> bool {
        match self {
            State::SynRcvd => false,
            State::Estab => true,
            State::FinWait1 => true,
            State::FinWait2 => true,
            State::Closing => true,
        }
    }
}
pub(crate) struct Connection {
    state: State,
    send: SendSequenceSpace,
    receive: ReceiveSequenceSpace,
    ip: etherparse::Ipv4Header,
    tcp: etherparse::TcpHeader,
}

/// State of the Send Sequence Space (RFC 793, S3.2 F4)
///
///```
///                    1         2          3          4
///               ----------|----------|----------|----------
///                      SND.UNA    SND.NXT    SND.UNA
///                                           +SND.WND
///
///         1 - old sequence numbers which have been acknowledged
///         2 - sequence numbers of unacknowledged data
///         3 - sequence numbers allowed for new data transmission
///         4 - future sequence numbers which are not yet allowed
///```

struct SendSequenceSpace {
    /// send unacknowledged
    una: u32,
    /// send next
    nxt: u32,
    /// send window
    wnd: u16,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgment number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: u32,
}

/// State of the Receive Sequence Space (RFC 793, S3.2 F5)
///```
///                        1          2          3
///                    ----------|----------|----------
///                           RCV.NXT    RCV.NXT
///                                     +RCV.WND
///
///         1 - old sequence numbers which have been acknowledged
///         2 - sequence numbers allowed for new reception
///         3 - future sequence numbers which are not yet allowed
///```

struct ReceiveSequenceSpace {
    /// receive next
    nxt: u32,
    /// receive window
    wnd: u16,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: u32,
}

impl Connection {
    pub(crate) fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];
        if !tcph.syn() {
            // only expected SYN packet
            return Ok(None);
        }
        let iss = 0;
        let wnd = 10;
        let mut c = Connection {
            state: State::SynRcvd,
            // decide on stuff we're sending them
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss,
                wnd,
                up: false,

                wl1: 0,
                wl2: 0,
            },
            // keep track of sender info
            receive: ReceiveSequenceSpace {
                irs: tcph.sequence_number(),
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(
                0,
                64,
                etherparse::IpNumber::Tcp as u8,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            ),
            tcp: etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), iss, wnd),
        };

        // need to start establishing connection
        c.tcp.syn = true;
        c.tcp.ack = true;
        c.write(nic, &[])?;

        Ok(Some(c))
    }

    fn write(&mut self, nic: &mut tun_tap::Iface, payload: &[u8]) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        self.tcp.sequence_number = self.send.nxt;
        self.tcp.acknowledgment_number = self.receive.nxt;

        let size = std::cmp::min(
            buf.len(),
            self.tcp.header_len() as usize + self.ip.header_len() as usize + payload.len(),
        );
        self.ip.set_payload_len(size);
        // Write out the headers.
        use std::io::Write;
        let mut unwritten = &mut buf[..];
        self.ip.write(&mut unwritten);
        self.tcp.write(&mut unwritten);
        let payload_bytes = unwritten.write(payload)?;
        let unwritten = unwritten.len();
        self.send.nxt = self.send.nxt.wrapping_add(payload_bytes as u32);
        if self.tcp.syn {
            self.send.nxt = self.send.nxt.wrapping_add(1);
            self.tcp.syn = false;
        }
        if self.tcp.fin {
            self.send.nxt = self.send.nxt.wrapping_add(1);
            self.tcp.fin = false;
        }
        nic.send(&buf[..buf.len() - unwritten])?;
        Ok(payload_bytes)
    }

    fn send_rst(&mut self, nic: &mut tun_tap::Iface) -> io::Result<()> {
        self.tcp.rst = true;
        // TODO: set sequence number to something useful
        // TODO: handle synchronized RST
        self.tcp.sequence_number = 0;
        self.tcp.acknowledgment_number = 0;
        self.write(nic, &[])?;
        Ok(())
    }

    pub(crate) fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {
        // valid segment check. okay if it acks at least one byte, which means that at least one
        // of the following is true:
        //
        // RCV.NXT <= SEG.SEQ < RCV.NXT+RCV.WND
        // RCV.NXT <= SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND
        // but remember wrapping!
        let seqn = tcph.sequence_number();
        let mut slen = data.len() as u32;
        if tcph.syn() {
            slen += 1;
        }
        if tcph.fin() {
            slen += 1;
        }
        let wend = self.receive.nxt.wrapping_add(self.receive.wnd as u32);
        if slen == 0 {
            // zero-length segment has separate rules
            if self.receive.wnd == 0 {
                if seqn != self.receive.nxt {
                    return Ok(());
                }
            } else if !is_between_wrapped(self.receive.nxt, seqn, wend) {
                return Ok(());
            }
        } else {
            if self.receive.wnd == 0 {
                return Ok(());
            } else if !is_between_wrapped(self.receive.nxt.wrapping_sub(1), seqn, wend)
                || !is_between_wrapped(
                    self.receive.nxt.wrapping_sub(1),
                    seqn.wrapping_add(slen - 1),
                    wend,
                )
            {
                return Ok(());
            }
        }
        self.receive.nxt = seqn.wrapping_add(slen);
        // TODO: if not expectable, send ACK
        // <SEQ=SND.NXT><ACK=RCV.NXT><CTL=ACK>

        if !tcph.ack() {
            return Ok(());
        }

        // first, check that sequence numbers are valid (RFC 793, S3.3)
        // acceptable ack check
        // SND.UNA < SEG.ACK =< SND.NXT
        // but remember wrapping!
        let ackn = tcph.acknowledgment_number();
        if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
            if !self.state.is_syn() {
                // according to Reset Generation, we should send a RST
                self.send_rst(nic)?;
            }
            return Ok(());
        }
        self.send.una = ackn;

        match self.state {
            State::SynRcvd => {
                // expect to get an ACK for our SYN
                if !tcph.ack() {
                    return Ok(());
                }
                // must have ACKed our SYN, since we detected at least one ACKed byte, and we
                // have sent one byte (the SYN)
                self.state = State::Estab;

                // now let's terminate the connection!
                // TODO: needs to be stored in the retransmission queue
                self.tcp.fin = true;
                self.write(nic, &[])?;
                self.state = State::FinWait1;
            }
            State::Estab => {
                unimplemented!()
            }
            State::FinWait1 => {
                if !tcph.fin() || !data.is_empty() {
                    unimplemented!()
                }

                // must have ACKed our FIN, since we detected at least one ACKed byte, and we
                // have sent one byte (the FIN)
                self.state = State::FinWait2;
            }
            State::FinWait2 => {
                if !tcph.fin() || !data.is_empty() {
                    unimplemented!()
                }

                self.tcp.fin = false;
                self.write(nic, &[])?;
                self.state = State::Closing;
            }
            State::Closing => {
                unimplemented!()
            }
        }

        Ok(())
    }
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    match start.cmp(&x) {
        std::cmp::Ordering::Less => {
            // we have:
            //     0 |---------S------X---------------| (wraparound)
            // X is between S and E (S < X < E) in these cases:
            //
            //     0 |---------S------X------E--------| (wraparound)
            //
            //     0 |------E---S------X--------------| (wraparound)
            //
            // but *not* in these cases:
            //     0 |---------S----E-----X-----------| (wraparound)
            //     0 |---------|------X---------------| (wraparound)
            //               ^-S+E
            //
            //     0 |-------------S-----|------------| (wraparound)
            //                     X+E-^
            // or, in other words, iff !(S <= E <= X)
            if end >= start && end <= x {
                return false;
            }
        }
        std::cmp::Ordering::Equal => return false,
        std::cmp::Ordering::Greater => {
            // we have the opposite of the above:
            //     0 |---------X------S---------------| (wraparound)
            // X is between S and E (S < X < E) *only* in this case:
            //     0 |---------X----E-----S-----------| (wraparound)
            // but *not* in these cases:
            //
            //
            //     0 |---------E---X------S-----------| (wraparound)
            //
            //     0 |---------|------S---------------| (wraparound)
            //                 ^-X+E
            //
            //     0 |-------------X-----|------------| (wraparound)
            //                       S+E-^
            // or, in other words, iff S < E < X
            if end < start && end > x {
            } else {
                return false;
            }
        }
    }
    true
}

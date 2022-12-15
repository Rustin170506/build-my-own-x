use std::io;

use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) enum State {
    Closed,
    Listen,
    SynRcvd,
    // Estab,
}

pub(crate) struct Connection {
    state: State,
    send: SendSequenceSpace,
    receive: ReceiveSequenceSpace,
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
        let mut c = Connection {
            state: State::SynRcvd,
            // decide on stuff we're sending them
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
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
        };

        // need to start establishing connection
        let mut syn_ack =
            etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), c.send.iss, 10);
        syn_ack.acknowledgment_number = c.receive.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;

        let ip = etherparse::Ipv4Header::new(
            syn_ack.header_len(),
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
        );
        // Write out the headers.
        let unwritten = {
            let mut unwritten = &mut buf[..];
            ip.write(&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len()
        };
        nic.send(&buf[..buf.len() - unwritten])?;
        Ok(Some(c))
    }

    pub(crate) fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {
        Ok(())
    }
}

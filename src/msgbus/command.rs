// Bitcoin protocol (BP) daemon node
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use super::*;


#[derive(Clone, Debug, Display)]
#[display_from(Debug)]
#[non_exhaustive]
pub enum Command {
    Okay,
    Ack,
    Success,
    Done,
    Failure,
    Query(Query),
}

impl TryFrom<Multipart> for Command {
    type Error = Error;

    fn try_from(multipart: Multipart) -> Result<Self, Self::Error> {
        let (cmd, args) = multipart.split_first()
            .ok_or(Error::MalformedRequest)
            .and_then(|(cmd_data, args)| {
                if cmd_data.len() != 2 {
                    Err(Error::MalformedCommand)?
                }
                let mut buf = [0u8; 2];
                buf.clone_from_slice(&cmd_data[0..2]);
                Ok((u16::from_be_bytes(buf), args))
            })?;

        Ok(match cmd {
            MSGID_OKAY => Command::Okay,
            MSGID_ACK => Command::Ack,
            MSGID_SUCCESS => Command::Success,
            MSGID_DONE => Command::Done,
            MSGID_FAILURE => Command::Failure,
            MSGID_QUERY => Command::Query(args.try_into()?),
            _ => Err(Error::UnknownCommand)?,
        })
    }
}

impl From<Command> for Multipart {
    fn from(command: Command) -> Self {
        use Command::*;

        match command {
            Okay => vec![zmq::Message::from(&MSGID_OKAY.to_be_bytes()[..])],
            Ack => vec![zmq::Message::from(&MSGID_ACK.to_be_bytes()[..])],
            Success => vec![zmq::Message::from(&MSGID_SUCCESS.to_be_bytes()[..])],
            Done => vec![zmq::Message::from(&MSGID_DONE.to_be_bytes()[..])],
            Failure => vec![zmq::Message::from(&MSGID_FAILURE.to_be_bytes()[..])],
            Query(query) => vec![
                zmq::Message::from(&MSGID_QUERY.to_be_bytes()[..]),
            ].into_iter()
                .chain(Multipart::from(query))
                .collect::<Multipart>(),
        }
    }
}

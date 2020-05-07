use super::event::Event;
use binread::{BinRead, BinResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    events: Vec<Event>,
}

// impl FromStream for Track {
//     fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf)?;

//         assert_eq!(b"MTrk", &buf);
//         let length = reader.read_u32::<BigEndian>()?;

//         let mut events = Vec::<Event>::new();

//         let end = reader.seek(SeekFrom::Current(0))? as u32 + length;
//         let mut running_status: Option<u8> = None;

//         while (reader.seek(SeekFrom::Current(0))? as u32) < end {
//             events.push(Event::from_stream_context(reader, &mut running_status)?);
//         }

//         Ok(Self { events })
//     }
// }

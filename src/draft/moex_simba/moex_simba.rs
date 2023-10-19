use super::types::*;
use crate::common;

pub struct SimbaDecoder;

impl SimbaDecoder {
    pub fn decode(reader: &mut common::ByteArrayReader) {
        let mdph = reader.read_as::<MarketDataPacketHeader>();
        println!("{:?}", mdph);

        if mdph.is_incremental() {
            Self::decode_incremental(reader);
        } else {
            Self::decode_snapshot(reader);
        }
    }

    fn decode_incremental(reader: &mut common::ByteArrayReader) {
        let iph = reader.read_as::<IncrementalPacketHeader>();
        println!("{:?}", iph);

        while reader.has_more() {
            let sbeh = reader.read_as::<SBEHeader>();
            println!("{:?}", sbeh);

            match sbeh.template_id {
                OrderUpdate::TEMPLATE_ID => {
                    let ou = reader.read_as::<OrderUpdate>();
                    println!("{:?}", ou);
                }
                OrderExecution::TEMPLATE_ID => {
                    let oe = reader.read_as::<OrderExecution>();
                    println!("{:?}", oe);
                }
                BestPrices::TEMPLATE_ID => {
                    let no_md_entries = reader.read_as::<GroupSize>();
                    reader.skip(
                        no_md_entries.block_length as usize * no_md_entries.num_in_group as usize,
                    );
                }
                SecurityMassStatus::TEMPLATE_ID => {
                    let no_md_entries = reader.read_as::<GroupSize2>();
                    reader.skip(
                        no_md_entries.block_length as usize * no_md_entries.num_in_group as usize,
                    );
                }
                _ => {
                    reader.skip(sbeh.block_length as usize);
                }
            }
        }
    }

    fn decode_snapshot(reader: &mut common::ByteArrayReader) {
        let sbeh = reader.read_as::<SBEHeader>();
        println!("{:?}", sbeh);

        match sbeh.template_id {
            OrderBookSnapshot::TEMPLATE_ID => {
                let obs = reader.read_as::<OrderBookSnapshot>();
                println!("{:?}", obs);

                for _ in 0..obs.no_md_entries.num_in_group {
                    let entry = reader.read_as::<OrderBookSnapshotEntry>();
                    println!("{:?}", entry);
                }
            }
            _ => {}
        }
    }
}

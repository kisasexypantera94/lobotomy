extern crate lobotomy;

use pcap_parser::*;
use std::fs::File;

#[test]
fn moex_simba_test() {
    let file = File::open("/Users/dvgr/dev/lobotomy/resources/2023-10-10.1849-1906.pcap").unwrap();
    let mut num_blocks = 0;
    let mut reader = create_reader(65536, file).unwrap();
    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    PcapBlockOwned::LegacyHeader(_) => {}
                    PcapBlockOwned::Legacy(b) => {
                        let mut reader = lobotomy::common::ByteArrayReader::new(b.data);
                        let ethernet = reader.read_as::<lobotomy::net::EthernetHeader>();
                        if ethernet.get_type() != lobotomy::net::EthernetHeader::IP {
                            println!("VLAN not supported: ethernet=[{:?}]", ethernet);
                            continue;
                        }

                        let ip = reader.read_as::<lobotomy::net::IpHeader>();
                        if ip.protocol != lobotomy::net::IpHeader::UDP {
                            println!("Only UDP supported: ip=[{:?}]", ip);
                            continue;
                        }

                        let _udp = reader.read_as::<lobotomy::net::UdpHeader>();
                        // lobotomy::draft::SimbaDecoder::decode(&mut reader);

                        let mdph =
                            reader.read_as::<lobotomy::draft::types::MarketDataPacketHeader>();
                        // println!("MDPH: {:?}", mdph);

                        if mdph.is_incremental() {
                            let _iph = reader
                                .read_as::<lobotomy::draft::types::IncrementalPacketHeader>();
                        }

                        let buf = moex_spectra_simba::ReadBuf::new(reader.as_slice());
                        let header =
                            moex_spectra_simba::MessageHeaderDecoder::default().wrap(buf, 0);

                        match header.template_id() {
                            moex_spectra_simba::best_prices_codec::SBE_TEMPLATE_ID => {
                                let best_prices_decoder =
                                    moex_spectra_simba::BestPricesDecoder::default().header(header);

                                let mut no_md_entries_decoder =
                                    best_prices_decoder.no_md_entries_decoder();

                                for i in 0..no_md_entries_decoder.count() {
                                    let mut mkt_bid_px_decoder =
                                        no_md_entries_decoder.mkt_bid_px_decoder();

                                    let mantissa = mkt_bid_px_decoder.mantissa().unwrap() as f64;
                                    let exponent = mkt_bid_px_decoder.exponent();
                                    let result = mantissa * 10_f64.powf(exponent as f64);
                                    println!("Price_{}: {}, {}", i, result, mantissa);

                                    no_md_entries_decoder = mkt_bid_px_decoder.parent().unwrap();
                                }
                            }
                            _ => (),
                        }
                    }

                    PcapBlockOwned::NG(_) => panic!("unexpected NG data"),
                }

                num_blocks += 1;
                reader.consume(offset);
            }
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete) => {
                reader.refill().unwrap();
            }
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }
    println!("num_blocks: {}", num_blocks);
}

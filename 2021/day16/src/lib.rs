use nom::{bits::complete as bits, IResult};

//
// [ packet header  ]
// [version][type_id]
// [1][2][3][1][2][3]
//
//
// Type IDs:
// - 4: "literal". Single 5-bit aligned binary number. [p][1][2][3][4]
//      Each word is prefixed with "1" except last, which is prefixed with "0"
//
// - ~4 "operator" Performs some calculation on one or more sub-packets contained within
//
//      [length type ID]
//      [      1       ]
//
//      "length type ID"
//      - 0 next 15 bits contain the total length in bits of the sub-packets
//      - 1 next 11 bits contain the number of sub-packets
//

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Packet {
    version: u8,
    content: Content,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Content {
    Literal(u64),
    Operator(Operator),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Operator {
    type_id: u8,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (input, version) = bits::take(3usize)(input)?;
        let (input, content) = Content::parse(input)?;
        Ok((input, Packet { version, content }))
    }

    pub fn version_sum(&self) -> u32 {
        match &self.content {
            Content::Literal(_) => self.version as u32,
            Content::Operator(op) => {
                op.sub_packets.iter().map(|x| x.version_sum()).sum::<u32>() + self.version as u32
            }
        }
    }

    pub fn evaluate(&self) -> u64 {
        match &self.content {
            Content::Literal(v) => *v as u64,
            Content::Operator(op) => match op.type_id {
                0/*sum*/ =>   {op.sub_packets.iter().map(|x|  x.evaluate()).sum::<u64>()}
                1/*product*/=>{op.sub_packets.iter().fold(1,|p,x| p*x.evaluate())}
                2/*min*/=>{op.sub_packets.iter().fold(u64::MAX,|p,x| p.min(x.evaluate()))}
                3/*max*/=>{op.sub_packets.iter().fold(u64::MIN,|p,x| p.max(x.evaluate()))}
                5/*gt*/=>{if op.sub_packets[0].evaluate()>op.sub_packets[1].evaluate(){1}else{0}}
                6/*lt*/=>{if op.sub_packets[0].evaluate()<op.sub_packets[1].evaluate(){1}else{0}}
                7/*eq*/=>{if op.sub_packets[0].evaluate()==op.sub_packets[1].evaluate(){1}else{0}}
                _ => panic!("bad input")  
            },
        }
    }
}

impl Content {
    pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Content> {
        let (mut input, type_id) = bits::take(3usize)(input)?;
        if type_id == 4 {
            let mut literal = 0u64;
            loop {
                let (new_input, chunk): (_, u64) = bits::take(5usize)(input)?;
                literal <<= 4;
                literal |= chunk & 0x0F;
                input = new_input;
                if (chunk & 0x10) == 0 {
                    return Ok((input, Content::Literal(literal)));
                }
            }
        } else {
            let (input, operator) = Operator::parse(input, type_id)?;
            Ok((input, Content::Operator(operator)))
        }
    }
}

impl Operator {
    pub fn parse(input: (&[u8], usize), type_id: u8) -> IResult<(&[u8], usize), Operator> {
        let (mut input, length_type_id): (_, u8) = bits::take(1usize)(input)?;
        let mut sub_packets = Vec::new();
        if length_type_id == 0 {
            let (new_input, subpackets_bit_length) = bits::take(15usize)(input)?;
            input = new_input;
            let start_input = input;
            while input_bit_diff(start_input, input) < subpackets_bit_length {
                let (new_input, sub_packet) = Packet::parse(input)?;
                input = new_input;
                sub_packets.push(sub_packet);
            }
        } else {
            let (new_input, subpacket_count) = bits::take(11usize)(input)?;
            input = new_input;
            for _ in 0..subpacket_count {
                let (new_input, sub_packet) = Packet::parse(input)?;
                input = new_input;
                sub_packets.push(sub_packet);
            }
        }

        Ok((
            input,
            Operator {
                type_id,
                sub_packets,
            },
        ))
    }
}

fn input_bit_diff(first: (&[u8], usize), second: (&[u8], usize)) -> usize {
    let first_pointer = first.0.as_ptr() as usize;
    let second_pointer = second.0.as_ptr() as usize;
    let bit_diff = (second_pointer - first_pointer) * 8;
    bit_diff + second.1 - first.1
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let bytes = hex::decode(&"D2FE28").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 6,
                content: Content::Literal(2021)
            }
        );

        let bytes = hex::decode(&"A0016C880162017C3686B18A3D4780").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn part1() {
        let bytes = hex::decode(&"620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.version_sum(), 897);
    }

    #[test]
    fn example2() {
        let bytes = hex::decode(&"C200B40A82").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 1 + 2);

        let bytes = hex::decode(&"04005AC33890").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 6 * 9);

        let bytes = hex::decode(&"880086C3E88112").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 7);

        let bytes = hex::decode(&"CE00C43D881120").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 9);

        let bytes = hex::decode(&"D8005AC2A8F0").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 1);

        let bytes = hex::decode(&"F600BC2D8F").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 0);

        let bytes = hex::decode(&"9C005AC2F8F0").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 0);

        let bytes = hex::decode(&"9C0141080250320F1802104A08").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 1);
    }
    #[test]
    fn part2() {
        let bytes = hex::decode(&"620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0").unwrap();
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.evaluate(), 9485076995911);
    }
}

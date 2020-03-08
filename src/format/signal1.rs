use super::{Format, FormatError};

use convert_base::Convert;

pub struct SignalFormat {}

pub const FORMAT_NAME: &str = "signal1";

fn clean_buffer(buf: &mut Vec<u8>) -> Result<(), FormatError> {
    let mut write_pos = 0;
    for read_pos in 0..buf.len() {
        let point = buf[read_pos];
        match point {
            65..=90 => {
                buf[write_pos] = point - 65;
                write_pos += 1;
            }
            10 | 13 | 32 => {}
            _ => return Err(FormatError::MalformedInput),
        }
    }
    buf.truncate(write_pos);
    Ok(())
}

impl Format for SignalFormat {
    fn unpack_input(&self, input: &mut Vec<u8>) -> Result<(), FormatError> {
        clean_buffer(input)?;

        let mut convert = Convert::new(26, 256);
        let buf = convert.convert::<u8, u8>(input);
        input.clear();
        input.extend_from_slice(&buf);
        Ok(())
    }

    fn pack_output(&self, output: &mut Vec<u8>) {
        let mut convert = Convert::new(256, 26);
        let buf = convert.convert::<u8, u8>(output);

        output.clear();
        for (idx, point) in buf.iter().enumerate() {
            if idx != 0 {
                if idx % 30 == 0 {
                    // output.push(13);
                    output.push(10);
                } else if idx % 5 == 0 {
                    output.push(32)
                }
            }
            output.push(point + 65);
        }
        output.push(10);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn all_bytes_unpacked() -> Vec<u8> {
        let mut result = Vec::with_capacity(256);
        for i in 0..=0xFF {
            result.push(i)
        }
        result
    }

    fn all_bytes_packed() -> Vec<u8> {
        "WXDHD JHJAW PCFGL XFPOY KIGIR GYDNH\n\
        EYZGS WHWGS ZMPMU SUCWS AYDFD UHMWS\n\
        QRFRX VBLUX BQBQJ GLRIT BTVKL LOHVS\n\
        OJLCK QKAZK IBVRC BLEBQ NZJLH KDKRK\n\
        MXGYY TRVQY DSAOA UQEEJ GQURP CDNGG\n\
        MMGRZ WXGYR PCNUG FREOU ZKECE QFRKH\n\
        JFVLU MXGDM EDRIZ BLIKT XOOAY ZGJQN\n\
        MWZVC MOYRH ERGXP OWHGT JOVUP HPKZD\n\
        ZJTPE CSRTZ KTYUU LICZG UEFHK BJFMT\n\
        SFNBQ TFERP HWPNA JYLBI BKXZV CWCUA\n\
        VQESV NIZRB VEJBE ROGQX CUOEB NWQVN\n\
        VDKJD GCQOW QLXCL MTQBU DPGTG UOZHC\n\
        INTUW SGURM IEEPR DSSCV HDJVN AVLLH\n\
        UNPGJ OIIUD MCABZ KWKIO TGQJY TOTOP\n\
        GRPGE CLFUF QWHQX J\n"
            .as_bytes()
            .to_vec()
    }

    #[test]
    fn can_pack_any_byte() {
        let mut output = all_bytes_unpacked();
        SignalFormat {}.pack_output(&mut output);
        assert_eq!(all_bytes_packed(), output);
    }

    #[test]
    fn can_unpack_any_byte_with_whitespace() {
        let mut input = all_bytes_packed();
        SignalFormat {}.unpack_input(&mut input).unwrap();
        assert_eq!(all_bytes_unpacked(), input)
    }
}

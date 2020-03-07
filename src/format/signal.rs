use super::{Format, FormatError};

use convert_base::Convert;

pub struct SignalFormat {}

pub const FORMAT_NAME: &str = "signal";

impl Format for SignalFormat {
    fn pack(&self, input: &[u8]) -> Vec<u8> {
        let mut convert = Convert::new(256, 26);
        let mut output = Vec::new();
        for (idx, point) in convert.convert::<u8, u8>(input).iter().enumerate() {
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
        output
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let mut buf = Vec::with_capacity(input.len());
        for point in input {
            match point {
                65..=90 => buf.push(point - 65),
                10 | 13 | 32 => {}
                _ => return Err(FormatError::MalformedInput),
            }
        }
        let mut convert = Convert::new(26, 256);
        Ok(convert.convert::<u8, u8>(&buf))
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
        let raw = all_bytes_unpacked();
        let packed = SignalFormat {}.pack(&raw);
        assert_eq!(all_bytes_packed(), packed);
    }

    #[test]
    fn can_unpack_any_byte_with_whitespace() {
        let raw = all_bytes_packed();
        let unpacked = SignalFormat {}.unpack(&raw).unwrap();
        assert_eq!(all_bytes_unpacked(), unpacked)
    }
}

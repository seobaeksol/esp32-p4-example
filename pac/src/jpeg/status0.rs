#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<Status0Spec>;
#[doc = "Field `BITSTREAM_EOF_VLD_CNT` reader - the valid bit count for last bitstream"]
pub type BitstreamEofVldCntR = crate::FieldReader;
#[doc = "Field `DCTOUT_ZZSCAN_ADDR` reader - the zig-zag read addr from dctout_ram"]
pub type DctoutZzscanAddrR = crate::FieldReader;
#[doc = "Field `QNRVAL_ZZSCAN_ADDR` reader - the zig-zag read addr from qnrval_ram"]
pub type QnrvalZzscanAddrR = crate::FieldReader;
#[doc = "Field `REG_STATE_YUV` reader - the state of jpeg fsm"]
pub type RegStateYuvR = crate::FieldReader;
impl R {
    #[doc = "Bits 11:16 - the valid bit count for last bitstream"]
    #[inline(always)]
    pub fn bitstream_eof_vld_cnt(&self) -> BitstreamEofVldCntR {
        BitstreamEofVldCntR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:22 - the zig-zag read addr from dctout_ram"]
    #[inline(always)]
    pub fn dctout_zzscan_addr(&self) -> DctoutZzscanAddrR {
        DctoutZzscanAddrR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - the zig-zag read addr from qnrval_ram"]
    #[inline(always)]
    pub fn qnrval_zzscan_addr(&self) -> QnrvalZzscanAddrR {
        QnrvalZzscanAddrR::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:31 - the state of jpeg fsm"]
    #[inline(always)]
    pub fn reg_state_yuv(&self) -> RegStateYuvR {
        RegStateYuvR::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status0Spec;
impl crate::RegisterSpec for Status0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for Status0Spec {}
#[doc = "`reset()` method sets STATUS0 to value 0"]
impl crate::Resettable for Status0Spec {}

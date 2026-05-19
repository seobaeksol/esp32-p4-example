#[doc = "Register `OUT_CRC_CLEAR_CH%s` reader"]
pub type R = crate::R<OutCrcClearChSpec>;
#[doc = "Register `OUT_CRC_CLEAR_CH%s` writer"]
pub type W = crate::W<OutCrcClearChSpec>;
#[doc = "Field `OUT_CRC_CLEAR_CH` reader - This register is used to clear ch0 of tx crc result"]
pub type OutCrcClearChR = crate::BitReader;
#[doc = "Field `OUT_CRC_CLEAR_CH` writer - This register is used to clear ch0 of tx crc result"]
pub type OutCrcClearChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    pub fn out_crc_clear_ch(&self) -> OutCrcClearChR {
        OutCrcClearChR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    pub fn out_crc_clear_ch(&mut self) -> OutCrcClearChW<'_, OutCrcClearChSpec> {
        OutCrcClearChW::new(self, 0)
    }
}
#[doc = "This register is used to clear ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_clear_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_clear_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCrcClearChSpec;
impl crate::RegisterSpec for OutCrcClearChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_clear_ch::R`](R) reader structure"]
impl crate::Readable for OutCrcClearChSpec {}
#[doc = "`write(|w| ..)` method takes [`out_crc_clear_ch::W`](W) writer structure"]
impl crate::Writable for OutCrcClearChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CRC_CLEAR_CH%s to value 0"]
impl crate::Resettable for OutCrcClearChSpec {}

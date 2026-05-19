#[doc = "Register `IN_CRC_CLEAR` reader"]
pub type R = crate::R<InCrcClearSpec>;
#[doc = "Register `IN_CRC_CLEAR` writer"]
pub type W = crate::W<InCrcClearSpec>;
#[doc = "Field `IN_CRC_CLEAR` reader - This register is used to clear ch0 of rx crc result"]
pub type InCrcClearR = crate::BitReader;
#[doc = "Field `IN_CRC_CLEAR` writer - This register is used to clear ch0 of rx crc result"]
pub type InCrcClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to clear ch0 of rx crc result"]
    #[inline(always)]
    pub fn in_crc_clear(&self) -> InCrcClearR {
        InCrcClearR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to clear ch0 of rx crc result"]
    #[inline(always)]
    pub fn in_crc_clear(&mut self) -> InCrcClearW<'_, InCrcClearSpec> {
        InCrcClearW::new(self, 0)
    }
}
#[doc = "This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCrcClearSpec;
impl crate::RegisterSpec for InCrcClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_clear::R`](R) reader structure"]
impl crate::Readable for InCrcClearSpec {}
#[doc = "`write(|w| ..)` method takes [`in_crc_clear::W`](W) writer structure"]
impl crate::Writable for InCrcClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CRC_CLEAR to value 0"]
impl crate::Resettable for InCrcClearSpec {}

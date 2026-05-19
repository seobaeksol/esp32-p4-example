#[doc = "Register `XTS_DESTINATION` reader"]
pub type R = crate::R<XtsDestinationSpec>;
#[doc = "Register `XTS_DESTINATION` writer"]
pub type W = crate::W<XtsDestinationSpec>;
#[doc = "Field `SPI_XTS_DESTINATION` reader - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SpiXtsDestinationR = crate::BitReader;
#[doc = "Field `SPI_XTS_DESTINATION` writer - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SpiXtsDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn spi_xts_destination(&self) -> SpiXtsDestinationR {
        SpiXtsDestinationR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn spi_xts_destination(&mut self) -> SpiXtsDestinationW<'_, XtsDestinationSpec> {
        SpiXtsDestinationW::new(self, 0)
    }
}
#[doc = "Manual Encryption destination register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_destination::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destination::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtsDestinationSpec;
impl crate::RegisterSpec for XtsDestinationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xts_destination::R`](R) reader structure"]
impl crate::Readable for XtsDestinationSpec {}
#[doc = "`write(|w| ..)` method takes [`xts_destination::W`](W) writer structure"]
impl crate::Writable for XtsDestinationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTS_DESTINATION to value 0"]
impl crate::Resettable for XtsDestinationSpec {}

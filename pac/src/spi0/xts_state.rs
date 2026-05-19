#[doc = "Register `XTS_STATE` reader"]
pub type R = crate::R<XtsStateSpec>;
#[doc = "Field `SPI_XTS_STATE` reader - This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi."]
pub type SpiXtsStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi."]
    #[inline(always)]
    pub fn spi_xts_state(&self) -> SpiXtsStateR {
        SpiXtsStateR::new((self.bits & 3) as u8)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtsStateSpec;
impl crate::RegisterSpec for XtsStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xts_state::R`](R) reader structure"]
impl crate::Readable for XtsStateSpec {}
#[doc = "`reset()` method sets XTS_STATE to value 0"]
impl crate::Resettable for XtsStateSpec {}

#[doc = "Register `AE_CTRL` reader"]
pub type R = crate::R<AeCtrlSpec>;
#[doc = "Register `AE_CTRL` writer"]
pub type W = crate::W<AeCtrlSpec>;
#[doc = "Field `AE_UPDATE` writer - write 1 to this bit triggers one statistic event"]
pub type AeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_SELECT` reader - this field configures ae input data source, 0: data from median, 1: data from gama"]
pub type AeSelectR = crate::BitReader;
#[doc = "Field `AE_SELECT` writer - this field configures ae input data source, 0: data from median, 1: data from gama"]
pub type AeSelectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - this field configures ae input data source, 0: data from median, 1: data from gama"]
    #[inline(always)]
    pub fn ae_select(&self) -> AeSelectR {
        AeSelectR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to this bit triggers one statistic event"]
    #[inline(always)]
    pub fn ae_update(&mut self) -> AeUpdateW<'_, AeCtrlSpec> {
        AeUpdateW::new(self, 0)
    }
    #[doc = "Bit 1 - this field configures ae input data source, 0: data from median, 1: data from gama"]
    #[inline(always)]
    pub fn ae_select(&mut self) -> AeSelectW<'_, AeCtrlSpec> {
        AeSelectW::new(self, 1)
    }
}
#[doc = "ae control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeCtrlSpec;
impl crate::RegisterSpec for AeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_ctrl::R`](R) reader structure"]
impl crate::Readable for AeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ae_ctrl::W`](W) writer structure"]
impl crate::Writable for AeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AE_CTRL to value 0"]
impl crate::Resettable for AeCtrlSpec {}

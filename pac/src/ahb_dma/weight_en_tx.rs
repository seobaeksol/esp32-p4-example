#[doc = "Register `WEIGHT_EN_TX` reader"]
pub type R = crate::R<WeightEnTxSpec>;
#[doc = "Register `WEIGHT_EN_TX` writer"]
pub type W = crate::W<WeightEnTxSpec>;
#[doc = "Field `WEIGHT_EN_TX` reader - Configures whether to enable weight arbitration for TX.\\\\0: Disable\\\\1: Enable\\\\"]
pub type WeightEnTxR = crate::BitReader;
#[doc = "Field `WEIGHT_EN_TX` writer - Configures whether to enable weight arbitration for TX.\\\\0: Disable\\\\1: Enable\\\\"]
pub type WeightEnTxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable weight arbitration for TX.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn weight_en_tx(&self) -> WeightEnTxR {
        WeightEnTxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable weight arbitration for TX.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn weight_en_tx(&mut self) -> WeightEnTxW<'_, WeightEnTxSpec> {
        WeightEnTxW::new(self, 0)
    }
}
#[doc = "TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeightEnTxSpec;
impl crate::RegisterSpec for WeightEnTxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_en_tx::R`](R) reader structure"]
impl crate::Readable for WeightEnTxSpec {}
#[doc = "`write(|w| ..)` method takes [`weight_en_tx::W`](W) writer structure"]
impl crate::Writable for WeightEnTxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT_EN_TX to value 0"]
impl crate::Resettable for WeightEnTxSpec {}

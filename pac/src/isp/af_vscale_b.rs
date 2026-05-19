#[doc = "Register `AF_VSCALE_B` reader"]
pub type R = crate::R<AfVscaleBSpec>;
#[doc = "Register `AF_VSCALE_B` writer"]
pub type W = crate::W<AfVscaleBSpec>;
#[doc = "Field `AF_BPOINT_B` reader - this field configures right coordinate of focus window b, must <= hnum-2"]
pub type AfBpointBR = crate::FieldReader<u16>;
#[doc = "Field `AF_BPOINT_B` writer - this field configures right coordinate of focus window b, must <= hnum-2"]
pub type AfBpointBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_TPOINT_B` reader - this field configures bottom coordinate of focus window b, must <= hnum-2"]
pub type AfTpointBR = crate::FieldReader<u16>;
#[doc = "Field `AF_TPOINT_B` writer - this field configures bottom coordinate of focus window b, must <= hnum-2"]
pub type AfTpointBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window b, must <= hnum-2"]
    #[inline(always)]
    pub fn af_bpoint_b(&self) -> AfBpointBR {
        AfBpointBR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window b, must <= hnum-2"]
    #[inline(always)]
    pub fn af_tpoint_b(&self) -> AfTpointBR {
        AfTpointBR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window b, must <= hnum-2"]
    #[inline(always)]
    pub fn af_bpoint_b(&mut self) -> AfBpointBW<'_, AfVscaleBSpec> {
        AfBpointBW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window b, must <= hnum-2"]
    #[inline(always)]
    pub fn af_tpoint_b(&mut self) -> AfTpointBW<'_, AfVscaleBSpec> {
        AfTpointBW::new(self, 16)
    }
}
#[doc = "v-scale of af window b register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_vscale_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_vscale_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfVscaleBSpec;
impl crate::RegisterSpec for AfVscaleBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_vscale_b::R`](R) reader structure"]
impl crate::Readable for AfVscaleBSpec {}
#[doc = "`write(|w| ..)` method takes [`af_vscale_b::W`](W) writer structure"]
impl crate::Writable for AfVscaleBSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_VSCALE_B to value 0x0001_0080"]
impl crate::Resettable for AfVscaleBSpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}

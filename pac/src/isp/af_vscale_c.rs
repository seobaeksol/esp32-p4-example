#[doc = "Register `AF_VSCALE_C` reader"]
pub type R = crate::R<AfVscaleCSpec>;
#[doc = "Register `AF_VSCALE_C` writer"]
pub type W = crate::W<AfVscaleCSpec>;
#[doc = "Field `AF_BPOINT_C` reader - this field configures right coordinate of focus window c, must <= hnum-2"]
pub type AfBpointCR = crate::FieldReader<u16>;
#[doc = "Field `AF_BPOINT_C` writer - this field configures right coordinate of focus window c, must <= hnum-2"]
pub type AfBpointCW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_TPOINT_C` reader - this field configures bottom coordinate of focus window c, must <= hnum-2"]
pub type AfTpointCR = crate::FieldReader<u16>;
#[doc = "Field `AF_TPOINT_C` writer - this field configures bottom coordinate of focus window c, must <= hnum-2"]
pub type AfTpointCW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window c, must <= hnum-2"]
    #[inline(always)]
    pub fn af_bpoint_c(&self) -> AfBpointCR {
        AfBpointCR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window c, must <= hnum-2"]
    #[inline(always)]
    pub fn af_tpoint_c(&self) -> AfTpointCR {
        AfTpointCR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window c, must <= hnum-2"]
    #[inline(always)]
    pub fn af_bpoint_c(&mut self) -> AfBpointCW<'_, AfVscaleCSpec> {
        AfBpointCW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window c, must <= hnum-2"]
    #[inline(always)]
    pub fn af_tpoint_c(&mut self) -> AfTpointCW<'_, AfVscaleCSpec> {
        AfTpointCW::new(self, 16)
    }
}
#[doc = "v-scale of af window c register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_vscale_c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_vscale_c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfVscaleCSpec;
impl crate::RegisterSpec for AfVscaleCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_vscale_c::R`](R) reader structure"]
impl crate::Readable for AfVscaleCSpec {}
#[doc = "`write(|w| ..)` method takes [`af_vscale_c::W`](W) writer structure"]
impl crate::Writable for AfVscaleCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_VSCALE_C to value 0x0001_0080"]
impl crate::Resettable for AfVscaleCSpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}

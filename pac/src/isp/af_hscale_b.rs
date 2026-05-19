#[doc = "Register `AF_HSCALE_B` reader"]
pub type R = crate::R<AfHscaleBSpec>;
#[doc = "Register `AF_HSCALE_B` writer"]
pub type W = crate::W<AfHscaleBSpec>;
#[doc = "Field `AF_RPOINT_B` reader - this field configures left coordinate of focus window b, must >= 2"]
pub type AfRpointBR = crate::FieldReader<u16>;
#[doc = "Field `AF_RPOINT_B` writer - this field configures left coordinate of focus window b, must >= 2"]
pub type AfRpointBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_LPOINT_B` reader - this field configures top coordinate of focus window b, must >= 2"]
pub type AfLpointBR = crate::FieldReader<u16>;
#[doc = "Field `AF_LPOINT_B` writer - this field configures top coordinate of focus window b, must >= 2"]
pub type AfLpointBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_b(&self) -> AfRpointBR {
        AfRpointBR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_b(&self) -> AfLpointBR {
        AfLpointBR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_b(&mut self) -> AfRpointBW<'_, AfHscaleBSpec> {
        AfRpointBW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_b(&mut self) -> AfLpointBW<'_, AfHscaleBSpec> {
        AfLpointBW::new(self, 16)
    }
}
#[doc = "h-scale of af window b register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfHscaleBSpec;
impl crate::RegisterSpec for AfHscaleBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_hscale_b::R`](R) reader structure"]
impl crate::Readable for AfHscaleBSpec {}
#[doc = "`write(|w| ..)` method takes [`af_hscale_b::W`](W) writer structure"]
impl crate::Writable for AfHscaleBSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_HSCALE_B to value 0x0001_0080"]
impl crate::Resettable for AfHscaleBSpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}

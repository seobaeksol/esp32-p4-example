#[doc = "Register `AWB_HSCALE` reader"]
pub type R = crate::R<AwbHscaleSpec>;
#[doc = "Register `AWB_HSCALE` writer"]
pub type W = crate::W<AwbHscaleSpec>;
#[doc = "Field `AWB_RPOINT` reader - this field configures awb window right coordinate"]
pub type AwbRpointR = crate::FieldReader<u16>;
#[doc = "Field `AWB_RPOINT` writer - this field configures awb window right coordinate"]
pub type AwbRpointW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_LPOINT` reader - this field configures awb window left coordinate"]
pub type AwbLpointR = crate::FieldReader<u16>;
#[doc = "Field `AWB_LPOINT` writer - this field configures awb window left coordinate"]
pub type AwbLpointW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures awb window right coordinate"]
    #[inline(always)]
    pub fn awb_rpoint(&self) -> AwbRpointR {
        AwbRpointR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures awb window left coordinate"]
    #[inline(always)]
    pub fn awb_lpoint(&self) -> AwbLpointR {
        AwbLpointR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures awb window right coordinate"]
    #[inline(always)]
    pub fn awb_rpoint(&mut self) -> AwbRpointW<'_, AwbHscaleSpec> {
        AwbRpointW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures awb window left coordinate"]
    #[inline(always)]
    pub fn awb_lpoint(&mut self) -> AwbLpointW<'_, AwbHscaleSpec> {
        AwbLpointW::new(self, 16)
    }
}
#[doc = "h-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_hscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_hscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbHscaleSpec;
impl crate::RegisterSpec for AwbHscaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_hscale::R`](R) reader structure"]
impl crate::Readable for AwbHscaleSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_hscale::W`](W) writer structure"]
impl crate::Writable for AwbHscaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_HSCALE to value 0x077f"]
impl crate::Resettable for AwbHscaleSpec {
    const RESET_VALUE: u32 = 0x077f;
}

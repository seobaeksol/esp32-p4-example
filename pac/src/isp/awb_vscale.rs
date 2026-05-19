#[doc = "Register `AWB_VSCALE` reader"]
pub type R = crate::R<AwbVscaleSpec>;
#[doc = "Register `AWB_VSCALE` writer"]
pub type W = crate::W<AwbVscaleSpec>;
#[doc = "Field `AWB_BPOINT` reader - this field configures awb window bottom coordinate"]
pub type AwbBpointR = crate::FieldReader<u16>;
#[doc = "Field `AWB_BPOINT` writer - this field configures awb window bottom coordinate"]
pub type AwbBpointW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_TPOINT` reader - this field configures awb window top coordinate"]
pub type AwbTpointR = crate::FieldReader<u16>;
#[doc = "Field `AWB_TPOINT` writer - this field configures awb window top coordinate"]
pub type AwbTpointW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures awb window bottom coordinate"]
    #[inline(always)]
    pub fn awb_bpoint(&self) -> AwbBpointR {
        AwbBpointR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures awb window top coordinate"]
    #[inline(always)]
    pub fn awb_tpoint(&self) -> AwbTpointR {
        AwbTpointR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures awb window bottom coordinate"]
    #[inline(always)]
    pub fn awb_bpoint(&mut self) -> AwbBpointW<'_, AwbVscaleSpec> {
        AwbBpointW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures awb window top coordinate"]
    #[inline(always)]
    pub fn awb_tpoint(&mut self) -> AwbTpointW<'_, AwbVscaleSpec> {
        AwbTpointW::new(self, 16)
    }
}
#[doc = "v-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_vscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_vscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbVscaleSpec;
impl crate::RegisterSpec for AwbVscaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_vscale::R`](R) reader structure"]
impl crate::Readable for AwbVscaleSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_vscale::W`](W) writer structure"]
impl crate::Writable for AwbVscaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_VSCALE to value 0x0437"]
impl crate::Resettable for AwbVscaleSpec {
    const RESET_VALUE: u32 = 0x0437;
}

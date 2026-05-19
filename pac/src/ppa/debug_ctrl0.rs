#[doc = "Register `DEBUG_CTRL0` reader"]
pub type R = crate::R<DebugCtrl0Spec>;
#[doc = "Register `DEBUG_CTRL0` writer"]
pub type W = crate::W<DebugCtrl0Spec>;
#[doc = "Field `DBG_REPLACE_SEL` reader - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
pub type DbgReplaceSelR = crate::FieldReader;
#[doc = "Field `DBG_REPLACE_SEL` writer - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
pub type DbgReplaceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
    #[inline(always)]
    pub fn dbg_replace_sel(&self) -> DbgReplaceSelR {
        DbgReplaceSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
    #[inline(always)]
    pub fn dbg_replace_sel(&mut self) -> DbgReplaceSelW<'_, DebugCtrl0Spec> {
        DbgReplaceSelW::new(self, 0)
    }
}
#[doc = "debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugCtrl0Spec;
impl crate::RegisterSpec for DebugCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_ctrl0::R`](R) reader structure"]
impl crate::Readable for DebugCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_ctrl0::W`](W) writer structure"]
impl crate::Writable for DebugCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CTRL0 to value 0"]
impl crate::Resettable for DebugCtrl0Spec {}

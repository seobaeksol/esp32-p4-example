#[doc = "Register `BLEND1_CLUT_DATA` reader"]
pub type R = crate::R<Blend1ClutDataSpec>;
#[doc = "Register `BLEND1_CLUT_DATA` writer"]
pub type W = crate::W<Blend1ClutDataSpec>;
#[doc = "Field `RDWR_WORD_BLEND1_CLUT` reader - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
pub type RdwrWordBlend1ClutR = crate::FieldReader<u32>;
#[doc = "Field `RDWR_WORD_BLEND1_CLUT` writer - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
pub type RdwrWordBlend1ClutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    pub fn rdwr_word_blend1_clut(&self) -> RdwrWordBlend1ClutR {
        RdwrWordBlend1ClutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    pub fn rdwr_word_blend1_clut(&mut self) -> RdwrWordBlend1ClutW<'_, Blend1ClutDataSpec> {
        RdwrWordBlend1ClutW::new(self, 0)
    }
}
#[doc = "CLUT sram data read/write register in foreground plane of blender\n\nYou can [`read`](crate::Reg::read) this register and get [`blend1_clut_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend1_clut_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Blend1ClutDataSpec;
impl crate::RegisterSpec for Blend1ClutDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend1_clut_data::R`](R) reader structure"]
impl crate::Readable for Blend1ClutDataSpec {}
#[doc = "`write(|w| ..)` method takes [`blend1_clut_data::W`](W) writer structure"]
impl crate::Writable for Blend1ClutDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND1_CLUT_DATA to value 0"]
impl crate::Resettable for Blend1ClutDataSpec {}

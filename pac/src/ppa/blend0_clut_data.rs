#[doc = "Register `BLEND0_CLUT_DATA` reader"]
pub type R = crate::R<Blend0ClutDataSpec>;
#[doc = "Register `BLEND0_CLUT_DATA` writer"]
pub type W = crate::W<Blend0ClutDataSpec>;
#[doc = "Field `RDWR_WORD_BLEND0_CLUT` reader - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode."]
pub type RdwrWordBlend0ClutR = crate::FieldReader<u32>;
#[doc = "Field `RDWR_WORD_BLEND0_CLUT` writer - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode."]
pub type RdwrWordBlend0ClutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    pub fn rdwr_word_blend0_clut(&self) -> RdwrWordBlend0ClutR {
        RdwrWordBlend0ClutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    pub fn rdwr_word_blend0_clut(&mut self) -> RdwrWordBlend0ClutW<'_, Blend0ClutDataSpec> {
        RdwrWordBlend0ClutW::new(self, 0)
    }
}
#[doc = "CLUT sram data read/write register in background plane of blender\n\nYou can [`read`](crate::Reg::read) this register and get [`blend0_clut_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend0_clut_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Blend0ClutDataSpec;
impl crate::RegisterSpec for Blend0ClutDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend0_clut_data::R`](R) reader structure"]
impl crate::Readable for Blend0ClutDataSpec {}
#[doc = "`write(|w| ..)` method takes [`blend0_clut_data::W`](W) writer structure"]
impl crate::Writable for Blend0ClutDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND0_CLUT_DATA to value 0"]
impl crate::Resettable for Blend0ClutDataSpec {}

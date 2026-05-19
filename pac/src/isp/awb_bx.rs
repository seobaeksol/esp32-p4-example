#[doc = "Register `AWB_BX` reader"]
pub type R = crate::R<AwbBxSpec>;
#[doc = "Register `AWB_BX` writer"]
pub type W = crate::W<AwbBxSpec>;
#[doc = "Field `AWB_X_BSIZE` reader - Configures every block x size, min number is 4"]
pub type AwbXBsizeR = crate::FieldReader<u16>;
#[doc = "Field `AWB_X_BSIZE` writer - Configures every block x size, min number is 4"]
pub type AwbXBsizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_X_START` reader - Configures first block start x address"]
pub type AwbXStartR = crate::FieldReader<u16>;
#[doc = "Field `AWB_X_START` writer - Configures first block start x address"]
pub type AwbXStartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures every block x size, min number is 4"]
    #[inline(always)]
    pub fn awb_x_bsize(&self) -> AwbXBsizeR {
        AwbXBsizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Configures first block start x address"]
    #[inline(always)]
    pub fn awb_x_start(&self) -> AwbXStartR {
        AwbXStartR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures every block x size, min number is 4"]
    #[inline(always)]
    pub fn awb_x_bsize(&mut self) -> AwbXBsizeW<'_, AwbBxSpec> {
        AwbXBsizeW::new(self, 0)
    }
    #[doc = "Bits 12:23 - Configures first block start x address"]
    #[inline(always)]
    pub fn awb_x_start(&mut self) -> AwbXStartW<'_, AwbBxSpec> {
        AwbXStartW::new(self, 12)
    }
}
#[doc = "awb window register in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_bx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_bx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbBxSpec;
impl crate::RegisterSpec for AwbBxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_bx::R`](R) reader structure"]
impl crate::Readable for AwbBxSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_bx::W`](W) writer structure"]
impl crate::Writable for AwbBxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_BX to value 0"]
impl crate::Resettable for AwbBxSpec {}

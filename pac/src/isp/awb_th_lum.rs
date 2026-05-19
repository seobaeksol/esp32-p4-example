#[doc = "Register `AWB_TH_LUM` reader"]
pub type R = crate::R<AwbThLumSpec>;
#[doc = "Register `AWB_TH_LUM` writer"]
pub type W = crate::W<AwbThLumSpec>;
#[doc = "Field `AWB_MIN_LUM` reader - this field configures lower threshold of r+g+b"]
pub type AwbMinLumR = crate::FieldReader<u16>;
#[doc = "Field `AWB_MIN_LUM` writer - this field configures lower threshold of r+g+b"]
pub type AwbMinLumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AWB_MAX_LUM` reader - this field configures upper threshold of r+g+b"]
pub type AwbMaxLumR = crate::FieldReader<u16>;
#[doc = "Field `AWB_MAX_LUM` writer - this field configures upper threshold of r+g+b"]
pub type AwbMaxLumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - this field configures lower threshold of r+g+b"]
    #[inline(always)]
    pub fn awb_min_lum(&self) -> AwbMinLumR {
        AwbMinLumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of r+g+b"]
    #[inline(always)]
    pub fn awb_max_lum(&self) -> AwbMaxLumR {
        AwbMaxLumR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - this field configures lower threshold of r+g+b"]
    #[inline(always)]
    pub fn awb_min_lum(&mut self) -> AwbMinLumW<'_, AwbThLumSpec> {
        AwbMinLumW::new(self, 0)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of r+g+b"]
    #[inline(always)]
    pub fn awb_max_lum(&mut self) -> AwbMaxLumW<'_, AwbThLumSpec> {
        AwbMaxLumW::new(self, 16)
    }
}
#[doc = "awb lum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_th_lum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_th_lum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbThLumSpec;
impl crate::RegisterSpec for AwbThLumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_th_lum::R`](R) reader structure"]
impl crate::Readable for AwbThLumSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_th_lum::W`](W) writer structure"]
impl crate::Writable for AwbThLumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_TH_LUM to value 0x02fd_0000"]
impl crate::Resettable for AwbThLumSpec {
    const RESET_VALUE: u32 = 0x02fd_0000;
}

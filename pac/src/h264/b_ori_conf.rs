#[doc = "Register `B_ORI_CONF` reader"]
pub type R = crate::R<BOriConfSpec>;
#[doc = "Register `B_ORI_CONF` writer"]
pub type W = crate::W<BOriConfSpec>;
#[doc = "Field `B_ORI_COLOR_SPACE` reader - Configures video B original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
pub type BOriColorSpaceR = crate::FieldReader;
#[doc = "Field `B_ORI_COLOR_SPACE` writer - Configures video B original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
pub type BOriColorSpaceW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures video B original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
    #[inline(always)]
    pub fn b_ori_color_space(&self) -> BOriColorSpaceR {
        BOriColorSpaceR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures video B original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
    #[inline(always)]
    pub fn b_ori_color_space(&mut self) -> BOriColorSpaceW<'_, BOriConfSpec> {
        BOriColorSpaceW::new(self, 0)
    }
}
#[doc = "Video B original picture configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_ori_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_ori_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOriConfSpec;
impl crate::RegisterSpec for BOriConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_ori_conf::R`](R) reader structure"]
impl crate::Readable for BOriConfSpec {}
#[doc = "`write(|w| ..)` method takes [`b_ori_conf::W`](W) writer structure"]
impl crate::Writable for BOriConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_ORI_CONF to value 0x04"]
impl crate::Resettable for BOriConfSpec {
    const RESET_VALUE: u32 = 0x04;
}

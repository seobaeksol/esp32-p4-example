#[doc = "Register `MODE_CFG` reader"]
pub type R = crate::R<ModeCfgSpec>;
#[doc = "Register `MODE_CFG` writer"]
pub type W = crate::W<ModeCfgSpec>;
#[doc = "Field `CMD_VIDEO_MODE` reader - NA"]
pub type CmdVideoModeR = crate::BitReader;
#[doc = "Field `CMD_VIDEO_MODE` writer - NA"]
pub type CmdVideoModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn cmd_video_mode(&self) -> CmdVideoModeR {
        CmdVideoModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn cmd_video_mode(&mut self) -> CmdVideoModeW<'_, ModeCfgSpec> {
        CmdVideoModeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeCfgSpec;
impl crate::RegisterSpec for ModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_cfg::R`](R) reader structure"]
impl crate::Readable for ModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_cfg::W`](W) writer structure"]
impl crate::Writable for ModeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE_CFG to value 0x01"]
impl crate::Resettable for ModeCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}

#[doc = "Register `GOP_CONF` reader"]
pub type R = crate::R<GopConfSpec>;
#[doc = "Register `GOP_CONF` writer"]
pub type W = crate::W<GopConfSpec>;
#[doc = "Field `DUAL_STREAM_MODE` reader - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
pub type DualStreamModeR = crate::BitReader;
#[doc = "Field `DUAL_STREAM_MODE` writer - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
pub type DualStreamModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOP_NUM` reader - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
pub type GopNumR = crate::FieldReader;
#[doc = "Field `GOP_NUM` writer - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
pub type GopNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
    #[inline(always)]
    pub fn dual_stream_mode(&self) -> DualStreamModeR {
        DualStreamModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
    #[inline(always)]
    pub fn gop_num(&self) -> GopNumR {
        GopNumR::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
    #[inline(always)]
    pub fn dual_stream_mode(&mut self) -> DualStreamModeW<'_, GopConfSpec> {
        DualStreamModeW::new(self, 0)
    }
    #[doc = "Bits 1:8 - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
    #[inline(always)]
    pub fn gop_num(&mut self) -> GopNumW<'_, GopConfSpec> {
        GopNumW::new(self, 1)
    }
}
#[doc = "GOP related configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gop_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gop_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GopConfSpec;
impl crate::RegisterSpec for GopConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gop_conf::R`](R) reader structure"]
impl crate::Readable for GopConfSpec {}
#[doc = "`write(|w| ..)` method takes [`gop_conf::W`](W) writer structure"]
impl crate::Writable for GopConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOP_CONF to value 0"]
impl crate::Resettable for GopConfSpec {}

#[doc = "Register `VID_PKT_SIZE` reader"]
pub type R = crate::R<VidPktSizeSpec>;
#[doc = "Register `VID_PKT_SIZE` writer"]
pub type W = crate::W<VidPktSizeSpec>;
#[doc = "Field `VID_PKT_SIZE` reader - NA"]
pub type VidPktSizeR = crate::FieldReader<u16>;
#[doc = "Field `VID_PKT_SIZE` writer - NA"]
pub type VidPktSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    pub fn vid_pkt_size(&self) -> VidPktSizeR {
        VidPktSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    pub fn vid_pkt_size(&mut self) -> VidPktSizeW<'_, VidPktSizeSpec> {
        VidPktSizeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_pkt_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_pkt_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidPktSizeSpec;
impl crate::RegisterSpec for VidPktSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_pkt_size::R`](R) reader structure"]
impl crate::Readable for VidPktSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_pkt_size::W`](W) writer structure"]
impl crate::Writable for VidPktSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_PKT_SIZE to value 0"]
impl crate::Resettable for VidPktSizeSpec {}

#[doc = "Register `HOST_SIZE_CTRL` reader"]
pub type R = crate::R<HostSizeCtrlSpec>;
#[doc = "Register `HOST_SIZE_CTRL` writer"]
pub type W = crate::W<HostSizeCtrlSpec>;
#[doc = "Field `CSI_HOST_CM_VNUM` reader - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CsiHostCmVnumR = crate::FieldReader<u16>;
#[doc = "Field `CSI_HOST_CM_VNUM` writer - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CsiHostCmVnumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CSI_HOST_CM_HNUM` reader - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CsiHostCmHnumR = crate::FieldReader<u16>;
#[doc = "Field `CSI_HOST_CM_HNUM` writer - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CsiHostCmHnumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_vnum(&self) -> CsiHostCmVnumR {
        CsiHostCmVnumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_hnum(&self) -> CsiHostCmHnumR {
        CsiHostCmHnumR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_vnum(&mut self) -> CsiHostCmVnumW<'_, HostSizeCtrlSpec> {
        CsiHostCmVnumW::new(self, 0)
    }
    #[doc = "Bits 12:23 - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_hnum(&mut self) -> CsiHostCmHnumW<'_, HostSizeCtrlSpec> {
        CsiHostCmHnumW::new(self, 12)
    }
}
#[doc = "CSI HOST color mode convert configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_size_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_size_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostSizeCtrlSpec;
impl crate::RegisterSpec for HostSizeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_size_ctrl::R`](R) reader structure"]
impl crate::Readable for HostSizeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_size_ctrl::W`](W) writer structure"]
impl crate::Writable for HostSizeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SIZE_CTRL to value 0"]
impl crate::Resettable for HostSizeCtrlSpec {}

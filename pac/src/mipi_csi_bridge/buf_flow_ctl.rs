#[doc = "Register `BUF_FLOW_CTL` reader"]
pub type R = crate::R<BufFlowCtlSpec>;
#[doc = "Register `BUF_FLOW_CTL` writer"]
pub type W = crate::W<BufFlowCtlSpec>;
#[doc = "Field `CSI_BUF_AFULL_THRD` reader - buffer almost full threshold."]
pub type CsiBufAfullThrdR = crate::FieldReader<u16>;
#[doc = "Field `CSI_BUF_AFULL_THRD` writer - buffer almost full threshold."]
pub type CsiBufAfullThrdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CSI_BUF_DEPTH` reader - buffer data count."]
pub type CsiBufDepthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - buffer almost full threshold."]
    #[inline(always)]
    pub fn csi_buf_afull_thrd(&self) -> CsiBufAfullThrdR {
        CsiBufAfullThrdR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - buffer data count."]
    #[inline(always)]
    pub fn csi_buf_depth(&self) -> CsiBufDepthR {
        CsiBufDepthR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - buffer almost full threshold."]
    #[inline(always)]
    pub fn csi_buf_afull_thrd(&mut self) -> CsiBufAfullThrdW<'_, BufFlowCtlSpec> {
        CsiBufAfullThrdW::new(self, 0)
    }
}
#[doc = "csi bridge buffer control.\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_flow_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_flow_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufFlowCtlSpec;
impl crate::RegisterSpec for BufFlowCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_flow_ctl::R`](R) reader structure"]
impl crate::Readable for BufFlowCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_flow_ctl::W`](W) writer structure"]
impl crate::Writable for BufFlowCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF_FLOW_CTL to value 0x07f8"]
impl crate::Resettable for BufFlowCtlSpec {
    const RESET_VALUE: u32 = 0x07f8;
}

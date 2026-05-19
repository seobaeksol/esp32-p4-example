#[doc = "Register `OUT_CH2_DBG_DATA_L` reader"]
pub type R = crate::R<OutCh2DbgDataLSpec>;
#[doc = "Register `OUT_CH2_DBG_DATA_L` writer"]
pub type W = crate::W<OutCh2DbgDataLSpec>;
#[doc = "Field `H264_OUT_CH2_DBG_DATA_L` reader - configures out channel 2 debug data bit 31-0"]
pub type H264OutCh2DbgDataLR = crate::FieldReader<u32>;
#[doc = "Field `H264_OUT_CH2_DBG_DATA_L` writer - configures out channel 2 debug data bit 31-0"]
pub type H264OutCh2DbgDataLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures out channel 2 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_data_l(&self) -> H264OutCh2DbgDataLR {
        H264OutCh2DbgDataLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures out channel 2 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_data_l(&mut self) -> H264OutCh2DbgDataLW<'_, OutCh2DbgDataLSpec> {
        H264OutCh2DbgDataLW::new(self, 0)
    }
}
#[doc = "out channel 2 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ch2_dbg_data_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_ch2_dbg_data_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCh2DbgDataLSpec;
impl crate::RegisterSpec for OutCh2DbgDataLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ch2_dbg_data_l::R`](R) reader structure"]
impl crate::Readable for OutCh2DbgDataLSpec {}
#[doc = "`write(|w| ..)` method takes [`out_ch2_dbg_data_l::W`](W) writer structure"]
impl crate::Writable for OutCh2DbgDataLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CH2_DBG_DATA_L to value 0"]
impl crate::Resettable for OutCh2DbgDataLSpec {}

#[doc = "Register `OUT_CH2_DBG_DATA_H` reader"]
pub type R = crate::R<OutCh2DbgDataHSpec>;
#[doc = "Register `OUT_CH2_DBG_DATA_H` writer"]
pub type W = crate::W<OutCh2DbgDataHSpec>;
#[doc = "Field `H264_OUT_CH2_DBG_DATA_H` reader - configures out channel 2 debug data bit 63-32"]
pub type H264OutCh2DbgDataHR = crate::FieldReader<u32>;
#[doc = "Field `H264_OUT_CH2_DBG_DATA_H` writer - configures out channel 2 debug data bit 63-32"]
pub type H264OutCh2DbgDataHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures out channel 2 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_data_h(&self) -> H264OutCh2DbgDataHR {
        H264OutCh2DbgDataHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures out channel 2 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_data_h(&mut self) -> H264OutCh2DbgDataHW<'_, OutCh2DbgDataHSpec> {
        H264OutCh2DbgDataHW::new(self, 0)
    }
}
#[doc = "out channel 2 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ch2_dbg_data_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_ch2_dbg_data_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCh2DbgDataHSpec;
impl crate::RegisterSpec for OutCh2DbgDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ch2_dbg_data_h::R`](R) reader structure"]
impl crate::Readable for OutCh2DbgDataHSpec {}
#[doc = "`write(|w| ..)` method takes [`out_ch2_dbg_data_h::W`](W) writer structure"]
impl crate::Writable for OutCh2DbgDataHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CH2_DBG_DATA_H to value 0"]
impl crate::Resettable for OutCh2DbgDataHSpec {}

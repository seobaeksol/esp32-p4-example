#[doc = "Register `LUT_WDATA` reader"]
pub type R = crate::R<LutWdataSpec>;
#[doc = "Register `LUT_WDATA` writer"]
pub type W = crate::W<LutWdataSpec>;
#[doc = "Field `LUT_WDATA` reader - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register"]
pub type LutWdataR = crate::FieldReader<u32>;
#[doc = "Field `LUT_WDATA` writer - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register"]
pub type LutWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register"]
    #[inline(always)]
    pub fn lut_wdata(&self) -> LutWdataR {
        LutWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register"]
    #[inline(always)]
    pub fn lut_wdata(&mut self) -> LutWdataW<'_, LutWdataSpec> {
        LutWdataW::new(self, 0)
    }
}
#[doc = "LUT write data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutWdataSpec;
impl crate::RegisterSpec for LutWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_wdata::R`](R) reader structure"]
impl crate::Readable for LutWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_wdata::W`](W) writer structure"]
impl crate::Writable for LutWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_WDATA to value 0"]
impl crate::Resettable for LutWdataSpec {}

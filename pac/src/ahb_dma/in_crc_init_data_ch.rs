#[doc = "Register `IN_CRC_INIT_DATA_CH%s` reader"]
pub type R = crate::R<InCrcInitDataChSpec>;
#[doc = "Register `IN_CRC_INIT_DATA_CH%s` writer"]
pub type W = crate::W<InCrcInitDataChSpec>;
#[doc = "Field `IN_CRC_INIT_DATA_CH` reader - This register is used to config ch0 of rx crc initial value"]
pub type InCrcInitDataChR = crate::FieldReader<u32>;
#[doc = "Field `IN_CRC_INIT_DATA_CH` writer - This register is used to config ch0 of rx crc initial value"]
pub type InCrcInitDataChW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to config ch0 of rx crc initial value"]
    #[inline(always)]
    pub fn in_crc_init_data_ch(&self) -> InCrcInitDataChR {
        InCrcInitDataChR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to config ch0 of rx crc initial value"]
    #[inline(always)]
    pub fn in_crc_init_data_ch(&mut self) -> InCrcInitDataChW<'_, InCrcInitDataChSpec> {
        InCrcInitDataChW::new(self, 0)
    }
}
#[doc = "This register is used to config ch%s crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_init_data_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_init_data_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCrcInitDataChSpec;
impl crate::RegisterSpec for InCrcInitDataChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_init_data_ch::R`](R) reader structure"]
impl crate::Readable for InCrcInitDataChSpec {}
#[doc = "`write(|w| ..)` method takes [`in_crc_init_data_ch::W`](W) writer structure"]
impl crate::Writable for InCrcInitDataChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CRC_INIT_DATA_CH%s to value 0xffff_ffff"]
impl crate::Resettable for InCrcInitDataChSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

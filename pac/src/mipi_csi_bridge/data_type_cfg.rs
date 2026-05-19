#[doc = "Register `DATA_TYPE_CFG` reader"]
pub type R = crate::R<DataTypeCfgSpec>;
#[doc = "Register `DATA_TYPE_CFG` writer"]
pub type W = crate::W<DataTypeCfgSpec>;
#[doc = "Field `DATA_TYPE_MIN` reader - the min value of data type used for pixel filter."]
pub type DataTypeMinR = crate::FieldReader;
#[doc = "Field `DATA_TYPE_MIN` writer - the min value of data type used for pixel filter."]
pub type DataTypeMinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATA_TYPE_MAX` reader - the max value of data type used for pixel filter."]
pub type DataTypeMaxR = crate::FieldReader;
#[doc = "Field `DATA_TYPE_MAX` writer - the max value of data type used for pixel filter."]
pub type DataTypeMaxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - the min value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_min(&self) -> DataTypeMinR {
        DataTypeMinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - the max value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_max(&self) -> DataTypeMaxR {
        DataTypeMaxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - the min value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_min(&mut self) -> DataTypeMinW<'_, DataTypeCfgSpec> {
        DataTypeMinW::new(self, 0)
    }
    #[doc = "Bits 8:13 - the max value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_max(&mut self) -> DataTypeMaxW<'_, DataTypeCfgSpec> {
        DataTypeMaxW::new(self, 8)
    }
}
#[doc = "pixel data type configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_type_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_type_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataTypeCfgSpec;
impl crate::RegisterSpec for DataTypeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_type_cfg::R`](R) reader structure"]
impl crate::Readable for DataTypeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`data_type_cfg::W`](W) writer structure"]
impl crate::Writable for DataTypeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_TYPE_CFG to value 0x2f18"]
impl crate::Resettable for DataTypeCfgSpec {
    const RESET_VALUE: u32 = 0x2f18;
}

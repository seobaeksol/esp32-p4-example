#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DATE` reader - need des"]
pub type DateR = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - need des"]
pub type DateW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `I2C_MST_CLK_EN` reader - need des"]
pub type I2cMstClkEnR = crate::BitReader;
#[doc = "Field `I2C_MST_CLK_EN` writer - need des"]
pub type I2cMstClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - need des"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - need des"]
    #[inline(always)]
    pub fn i2c_mst_clk_en(&self) -> I2cMstClkEnR {
        I2cMstClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - need des"]
    #[inline(always)]
    pub fn date(&mut self) -> DateW<'_, DateSpec> {
        DateW::new(self, 0)
    }
    #[doc = "Bit 28 - need des"]
    #[inline(always)]
    pub fn i2c_mst_clk_en(&mut self) -> I2cMstClkEnW<'_, DateSpec> {
        I2cMstClkEnW::new(self, 28)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x0230_4230"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x0230_4230;
}

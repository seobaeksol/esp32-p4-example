#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "Field `CS0_DIS` reader - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type Cs0DisR = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type Cs0DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1_DIS` reader - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type Cs1DisR = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type Cs1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CkIdleEdgeR = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CkIdleEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CsKeepActiveR = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CsKeepActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> Cs0DisR {
        Cs0DisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> Cs1DisR {
        Cs1DisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CkIdleEdgeR {
        CkIdleEdgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CsKeepActiveR {
        CsKeepActiveR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> Cs0DisW<'_, MiscSpec> {
        Cs0DisW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> Cs1DisW<'_, MiscSpec> {
        Cs1DisW::new(self, 1)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CkIdleEdgeW<'_, MiscSpec> {
        CkIdleEdgeW::new(self, 9)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CsKeepActiveW<'_, MiscSpec> {
        CsKeepActiveW::new(self, 10)
    }
}
#[doc = "SPI1 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MiscSpec {
    const RESET_VALUE: u32 = 0x02;
}

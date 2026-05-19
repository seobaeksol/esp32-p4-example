#[doc = "Register `FLASH_SUS_CMD` reader"]
pub type R = crate::R<FlashSusCmdSpec>;
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub type W = crate::W<FlashSusCmdSpec>;
#[doc = "Field `FLASH_PES_COMMAND` reader - Program/Erase suspend command."]
pub type FlashPesCommandR = crate::FieldReader<u16>;
#[doc = "Field `FLASH_PES_COMMAND` writer - Program/Erase suspend command."]
pub type FlashPesCommandW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WAIT_PESR_COMMAND` reader - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type WaitPesrCommandR = crate::FieldReader<u16>;
#[doc = "Field `WAIT_PESR_COMMAND` writer - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type WaitPesrCommandW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FlashPesCommandR {
        FlashPesCommandR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&self) -> WaitPesrCommandR {
        WaitPesrCommandR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&mut self) -> FlashPesCommandW<'_, FlashSusCmdSpec> {
        FlashPesCommandW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&mut self) -> WaitPesrCommandW<'_, FlashSusCmdSpec> {
        WaitPesrCommandW::new(self, 16)
    }
}
#[doc = "SPI1 flash suspend command register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSusCmdSpec;
impl crate::RegisterSpec for FlashSusCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_cmd::R`](R) reader structure"]
impl crate::Readable for FlashSusCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_cmd::W`](W) writer structure"]
impl crate::Writable for FlashSusCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0x0005_7575"]
impl crate::Resettable for FlashSusCmdSpec {
    const RESET_VALUE: u32 = 0x0005_7575;
}

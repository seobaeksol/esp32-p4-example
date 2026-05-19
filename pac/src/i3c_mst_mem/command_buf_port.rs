#[doc = "Register `COMMAND_BUF_PORT` reader"]
pub type R = crate::R<CommandBufPortSpec>;
#[doc = "Register `COMMAND_BUF_PORT` writer"]
pub type W = crate::W<CommandBufPortSpec>;
#[doc = "Field `REG_COMMAND` reader - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
pub type RegCommandR = crate::FieldReader<u32>;
#[doc = "Field `REG_COMMAND` writer - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
pub type RegCommandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
    #[inline(always)]
    pub fn reg_command(&self) -> RegCommandR {
        RegCommandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
    #[inline(always)]
    pub fn reg_command(&mut self) -> RegCommandW<'_, CommandBufPortSpec> {
        RegCommandW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`command_buf_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command_buf_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandBufPortSpec;
impl crate::RegisterSpec for CommandBufPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command_buf_port::R`](R) reader structure"]
impl crate::Readable for CommandBufPortSpec {}
#[doc = "`write(|w| ..)` method takes [`command_buf_port::W`](W) writer structure"]
impl crate::Writable for CommandBufPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND_BUF_PORT to value 0"]
impl crate::Resettable for CommandBufPortSpec {}

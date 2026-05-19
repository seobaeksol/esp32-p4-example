#[doc = "Register `IBI_NOTIFY_CTRL` reader"]
pub type R = crate::R<IbiNotifyCtrlSpec>;
#[doc = "Register `IBI_NOTIFY_CTRL` writer"]
pub type W = crate::W<IbiNotifyCtrlSpec>;
#[doc = "Field `REG_NOTIFY_SIR_REJECTED` reader - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
pub type RegNotifySirRejectedR = crate::BitReader;
#[doc = "Field `REG_NOTIFY_SIR_REJECTED` writer - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
pub type RegNotifySirRejectedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
    #[inline(always)]
    pub fn reg_notify_sir_rejected(&self) -> RegNotifySirRejectedR {
        RegNotifySirRejectedR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
    #[inline(always)]
    pub fn reg_notify_sir_rejected(&mut self) -> RegNotifySirRejectedW<'_, IbiNotifyCtrlSpec> {
        RegNotifySirRejectedW::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ibi_notify_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibi_notify_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbiNotifyCtrlSpec;
impl crate::RegisterSpec for IbiNotifyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_notify_ctrl::R`](R) reader structure"]
impl crate::Readable for IbiNotifyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ibi_notify_ctrl::W`](W) writer structure"]
impl crate::Writable for IbiNotifyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IBI_NOTIFY_CTRL to value 0"]
impl crate::Resettable for IbiNotifyCtrlSpec {}

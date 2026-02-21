import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { BullModule } from '@nestjs/bullmq';

import { NotificationEntity } from './entities/notification.entity';
import { NotificationTemplateEntity } from './entities/notification-template.entity';

import { NotificationsGateway } from './gateways/notifications.gateway';

import { SmsProvider } from './providers/sms.provider';
import { PushProvider } from './providers/push.provider';
import { EmailProvider } from './providers/email.provider';
import { InAppProvider } from './providers/in-app.provider';

import { NotificationProcessor } from './processors/notification.processor';
import { NotificationsService } from './notifications.service';
import { NotificationsController } from './notifications.controller';

@Module({
  imports: [
    TypeOrmModule.forFeature([NotificationEntity, NotificationTemplateEntity]),
    BullModule.registerQueue({
      name: 'notifications',
    }),
  ],
  controllers: [NotificationsController],
  providers: [
    // Providers
    SmsProvider,
    PushProvider,
    EmailProvider,
    InAppProvider,

    // Gateways
    NotificationsGateway,

    // Processors
    NotificationProcessor,

    // Service
    NotificationsService,
  ],
  exports: [NotificationsService],
})
export class NotificationsModule {}

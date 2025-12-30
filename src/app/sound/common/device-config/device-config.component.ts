import {
  ChangeDetectionStrategy,
  Component,
  input,
  model,
} from '@angular/core';
import {
  PwDeviceProfile,
  PwDeviceRoute,
} from '../../../reducers/pipewire/pipewire.schema';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-device-config',
  templateUrl: './device-config.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule],
})
export class DeviceConfigComponent {
  readonly enumRoutes = input<PwDeviceRoute[]>([]);
  readonly enumProfiles = input<PwDeviceProfile[]>([]);

  readonly route = model<number>(-1);
  readonly profile = model<number>(-1);
}

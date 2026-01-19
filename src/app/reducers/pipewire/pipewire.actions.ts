import { createActionGroup, props } from '@ngrx/store';
import {
  PwNode,
  PwNodeProps,
  PwDevice,
  PwDeviceRoute,
  PwDeviceProfile,
  PwDefault,
  PwEnumRoutes,
} from './pipewire.schema';

export const PipewireActions = createActionGroup({
  source: 'Pipewire',
  events: {
    'Node added': props<{ node: PwNode }>(),
    'Node removed': props<{ node: number }>(),
    'Node props set': props<{ props: PwNodeProps }>(),
    'Device added': props<{ device: PwDevice }>(),
    'Enum routes added': props<{
      deviceId: number;
      enumRoutes: PwEnumRoutes;
    }>(),
    'Enum profiles added': props<{
      deviceId: number;
      enumProfiles: Record<number, PwDeviceProfile>;
    }>(),
    'Route added': props<{ route: PwDeviceRoute }>(),
    'Profile added': props<{ profile: PwDeviceProfile }>(),
    'Default sink set': props<{ defaultSink: PwDefault }>(),
    'Default source set': props<{ defaultSource: PwDefault }>(),

    'Change device volume': props<{
      id: number;
      volume: number;
      routeType: 'input' | 'output';
    }>(),
    'Mute device': props<{
      id: number;
      routeType: 'input' | 'output';
    }>(),
    'Unmute device': props<{
      id: number;
      routeType: 'input' | 'output';
    }>(),
    'Set default sink': props<{ id: number }>(),
    'Set default source': props<{ id: number }>(),
    'Set device route': props<{
      id: number;
      routeType: 'input' | 'output';
      routeIndex: number;
    }>(),
    'Set device profile': props<{ id: number; profileIndex: number }>(),
    'Change node volume': props<{
      id: number;
      volume: number;
    }>(),
    'Mute node': props<{
      id: number;
    }>(),
    'Unmute node': props<{
      id: number;
    }>(),
  },
});

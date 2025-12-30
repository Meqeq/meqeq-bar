import { createFeature, createReducer, on } from '@ngrx/store';
import { PipewireActions } from './pipewire.actions';
import {
  PwDeviceExtended,
  PwRouteDirection,
  PwNodeExtended,
} from './pipewire.schema';

export const pipewireFeatureKey = 'pipewire';

export interface State {
  nodes: Record<number, PwNodeExtended>;
  devices: Record<number, PwDeviceExtended>;
  defaultSinkName: string;
  defaultSourceName: string;
}

export const initialState: State = {
  nodes: {},
  devices: {},
  defaultSinkName: '',
  defaultSourceName: '',
};

export const reducer = createReducer(
  initialState,
  on(PipewireActions.nodeAdded, (state, { node }) => ({
    ...state,
    nodes: {
      ...state.nodes,
      [node.id]: { ...node, props: undefined },
    },
  })),
  on(PipewireActions.nodeRemoved, (state, { node }) => {
    const { [node]: ignored, ...newNodes } = state.nodes;

    return {
      ...state,
      nodes: newNodes,
    };
  }),
  on(PipewireActions.nodePropsSet, (state, { props }) => {
    const node = state.nodes[props.id];

    if (!node) return state;

    return {
      ...state,
      nodes: {
        ...state.nodes,
        [node.id]: {
          ...node,
          props,
        },
      },
    };
  }),
  on(PipewireActions.deviceAdded, (state, { device }) => ({
    ...state,
    devices: {
      ...state.devices,
      [device.id]: {
        ...device,
        route: {
          input: undefined,
          output: undefined,
        },
        enumRoutes: {
          [PwRouteDirection.Input]: {},
          [PwRouteDirection.Output]: {},
          [PwRouteDirection.Unknown]: {},
        },
        profile: undefined,
        enumProfiles: {},
      },
    },
  })),
  on(PipewireActions.enumRoutesAdded, (state, { deviceId, enumRoutes }) => {
    const device = state.devices[deviceId];

    if (!device) return state;

    return {
      ...state,
      devices: {
        ...state.devices,
        [deviceId]: {
          ...device,
          enumRoutes,
        },
      },
    };
  }),
  on(PipewireActions.routeAdded, (state, { route }) => {
    if (route.direction === 'unknown') return state;

    const device = state.devices[route.deviceId];

    if (!device) return state;

    return {
      ...state,
      devices: {
        ...state.devices,
        [route.deviceId]: {
          ...device,
          route: {
            ...device.route,
            [route.direction]: route,
          },
        },
      },
    };
  }),
  on(PipewireActions.enumProfilesAdded, (state, { deviceId, enumProfiles }) => {
    const device = state.devices[deviceId];

    if (!device) return state;

    return {
      ...state,
      devices: {
        ...state.devices,
        [deviceId]: {
          ...device,
          enumProfiles,
        },
      },
    };
  }),
  on(PipewireActions.profileAdded, (state, { profile }) => {
    const device = state.devices[profile.deviceId];

    if (!device) return state;

    return {
      ...state,
      devices: {
        ...state.devices,
        [device.id]: {
          ...device,
          profile,
        },
      },
    };
  }),
  on(PipewireActions.defaultSinkSet, (state, { defaultSink }) => ({
    ...state,
    defaultSinkName: defaultSink.name,
  })),
  on(PipewireActions.defaultSourceSet, (state, { defaultSource }) => ({
    ...state,
    defaultSourceName: defaultSource.name,
  })),
);

export const pipewireFeature = createFeature({
  name: pipewireFeatureKey,
  reducer,
});

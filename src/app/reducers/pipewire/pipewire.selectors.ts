import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromPipewire from './pipewire.reducer';
import { PwNodeClass } from './pipewire.schema';

export const selectPipewireState = createFeatureSelector<fromPipewire.State>(
  fromPipewire.pipewireFeatureKey,
);

export const selectDevices = createSelector(selectPipewireState, (state) => {
  return state.devices;
});

export const selectDevicesList = createSelector(selectDevices, (devices) =>
  Object.values(devices),
);

export const selectNodes = createSelector(selectPipewireState, (state) => {
  return state.nodes;
});

const selectDefaultSinkName = createSelector(selectPipewireState, (state) => {
  return state.defaultSinkName;
});

const selectDefaultSourceName = createSelector(selectPipewireState, (state) => {
  return state.defaultSourceName;
});

const selectDefaultSinkNode = createSelector(
  selectNodes,
  selectDefaultSinkName,
  (nodes, defaultSinkName) => {
    return Object.values(nodes).find((node) => node.name === defaultSinkName);
  },
);

const selectDefaultSourceNode = createSelector(
  selectNodes,
  selectDefaultSourceName,
  (nodes, defaultSourceName) => {
    return Object.values(nodes).find((node) => node.name === defaultSourceName);
  },
);

export const selectDefaultSink = createSelector(
  selectDevices,
  selectDefaultSinkNode,
  (devices, defaultNode) => {
    if (!defaultNode) return undefined;

    return devices[defaultNode.deviceId];
  },
);

export const selectDefaultSource = createSelector(
  selectDevices,
  selectDefaultSourceNode,
  (devices, defaultNode) => {
    if (!defaultNode) return undefined;

    return devices[defaultNode.deviceId];
  },
);

export const selectPlaybacks = createSelector(selectNodes, (nodes) => {
  return Object.values(nodes).filter(
    (node) => node.class === PwNodeClass.Playback,
  );
});

export const selectRecordings = createSelector(selectNodes, (nodes) => {
  return Object.values(nodes).filter(
    (node) => node.class === PwNodeClass.Recording,
  );
});

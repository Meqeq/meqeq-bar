export interface PipeWireMetadata {
  id: number;
  type: string;
  defaultSink: string;
  defaultSource: string;
}

export interface PipeWireNode {
  id: number;
  type: string;
  class: string;
  nick: string;
  description: string;
  name: string;
  muted: boolean;
  volume: number;
}

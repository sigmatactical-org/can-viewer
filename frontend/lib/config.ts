import type { CanViewerConfig, CanFrame, DbcInfo, MessageInfo } from './types';

/** Default configuration for CAN Viewer */
export const defaultConfig: Required<CanViewerConfig> = {
  appName: 'CAN Viewer',
  showDbcTab: true,
  showLiveTab: true,
  showMdf4Tab: true,
  showAboutTab: true,
  initialTab: 'dbc',
  autoScroll: true,
  maxFrames: 10000,
  maxSignals: 10000,
};

/** Filter state for CAN frames */
export interface Filters {
  timeMin: number | null;
  timeMax: number | null;
  canIds: number[] | null;
  messages: string[] | null;
  signals: string[] | null;
  dataPattern: string | null;
  channel: string | null;
  matchStatus: 'all' | 'matched' | 'unmatched';
}

/** Create empty filter state */
export function createEmptyFilters(): Filters {
  return {
    timeMin: null,
    timeMax: null,
    canIds: null,
    messages: null,
    signals: null,
    dataPattern: null,
    channel: null,
    matchStatus: 'all',
  };
}

/** Parse data pattern (hex bytes with wildcards, e.g., "01 ?? FF") */
export function parseDataPattern(input: string): string | null {
  const trimmed = input.trim().toUpperCase();
  if (!trimmed) return null;
  return trimmed;
}

/** Check if data matches pattern (supports ?? as wildcard) */
export function matchDataPattern(data: number[], pattern: string): boolean {
  const patternBytes = pattern.split(/\s+/);
  if (patternBytes.length > data.length) return false;

  for (let i = 0; i < patternBytes.length; i++) {
    const p = patternBytes[i];
    if (p === '??' || p === 'XX') continue;
    const expected = parseInt(p, 16);
    if (isNaN(expected) || data[i] !== expected) return false;
  }
  return true;
}

/** Parse CAN IDs from comma-separated hex string */
export function parseCanIds(input: string): number[] | null {
  const trimmed = input.trim();
  if (!trimmed) return null;

  const ids = trimmed
    .split(',')
    .map(s => s.trim())
    .filter(s => s.length > 0)
    .map(s => parseInt(s, 16))
    .filter(n => !isNaN(n));

  return ids.length > 0 ? ids : null;
}

/** Parse message names from comma-separated string */
export function parseMessageNames(input: string): string[] | null {
  const trimmed = input.trim().toLowerCase();
  if (!trimmed) return null;

  const names = trimmed
    .split(',')
    .map(s => s.trim().toLowerCase())
    .filter(s => s.length > 0);

  return names.length > 0 ? names : null;
}

/** Count active filters (time range counts as one filter) */
export function countActiveFilters(filters: Filters): number {
  let count = 0;
  if (filters.timeMin !== null || filters.timeMax !== null) count++;
  if (filters.canIds?.length) count++;
  if (filters.messages?.length) count++;
  if (filters.signals?.length) count++;
  if (filters.dataPattern) count++;
  if (filters.channel) count++;
  if (filters.matchStatus !== 'all') count++;
  return count;
}

/** Get message info from DBC by CAN ID */
function getMessageInfo(dbcInfo: DbcInfo | null, canId: number): MessageInfo | null {
  if (!dbcInfo?.messages) return null;
  return dbcInfo.messages.find(m => m.id === canId) || null;
}

/** Filter frames based on filter state and optional DBC info */
export function filterFrames(frames: CanFrame[], filters: Filters, dbcInfo: DbcInfo | null = null): CanFrame[] {
  return frames.filter(frame => {
    // Time range filter
    if (filters.timeMin !== null && frame.timestamp < filters.timeMin) return false;
    if (filters.timeMax !== null && frame.timestamp > filters.timeMax) return false;

    // CAN ID filter
    if (filters.canIds?.length && !filters.canIds.includes(frame.can_id)) return false;

    // Channel filter
    if (filters.channel) {
      const ch = filters.channel.toLowerCase();
      if (!frame.channel.toLowerCase().includes(ch)) return false;
    }

    // Data pattern filter
    if (filters.dataPattern && !matchDataPattern(frame.data, filters.dataPattern)) return false;

    // Get message info for DBC-related filters
    const msgInfo = getMessageInfo(dbcInfo, frame.can_id);
    const hasMatch = msgInfo !== null;

    // Match status filter
    if (filters.matchStatus === 'matched' && !hasMatch) return false;
    if (filters.matchStatus === 'unmatched' && hasMatch) return false;

    // Message name filter
    if (filters.messages?.length) {
      if (!hasMatch) return false;
      const msgName = msgInfo.name.toLowerCase();
      if (!filters.messages.some(m => msgName.includes(m))) return false;
    }

    // Signal name filter
    if (filters.signals?.length) {
      if (!hasMatch) return false;
      const signalNames = msgInfo.signals.map(s => s.name.toLowerCase());
      if (!filters.signals.some(s => signalNames.some(sn => sn.includes(s)))) return false;
    }

    return true;
  });
}

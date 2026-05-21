import achievementsData from '../data/generated/achievements.json';
import encountersData from '../data/generated/encounters.json';
import endingsData from '../data/generated/endings.json';
import itemsData from '../data/generated/items.json';
import locationsData from '../data/generated/locations.json';
import secretsData from '../data/generated/secrets.example.json';
import type { EncounterData, EndingData, LocationData, PublicSecret } from './types';

export const locations = locationsData as LocationData[];
export const encounters = encountersData as EncounterData[];
export const endings = endingsData as EndingData[];
export const publicSecrets = secretsData as PublicSecret[];
export const items = itemsData;
export const achievements = achievementsData;

export const locationsById = new Map(locations.map((location) => [location.id, location]));
export const publicSecretsById = new Map(publicSecrets.map((secret) => [secret.id, secret]));

// src/utils/validators/common.ts
export const validators = {
  required: (value: any) => {
    return value !== undefined && value !== null && value !== '';
  },
  email: (email: string) => {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
  },
  url: (url: string) => {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  },
  min: (value: number, min: number) => {
    return value >= min;
  },
  max: (value: number, max: number) => {
    return value <= max;
  }
};

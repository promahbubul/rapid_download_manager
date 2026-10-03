const basePath = process.env.NEXT_PUBLIC_BASE_PATH || "";

/**
 * Returns the absolute asset URL taking into account any configured Next.js basePath.
 */
export function assetUrl(path: string): string {
  if (!path.startsWith("/")) return path;
  if (!basePath) return path;
  return `${basePath}${path}`;
}

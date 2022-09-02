// Gets rid of typescript errors when importing png files
declare module "*.png" {
  const value: string;
  export default value;
}

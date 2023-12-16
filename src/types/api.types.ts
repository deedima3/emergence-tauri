type ApiError = {
    code: number;
    msg: string;
  }
  
  type ApiResponse<T> = {
    data: T;
    error: ApiError;
  }
  
  type ApiDataResponse<ResponseType> = {
    [key: string]: ResponseType;
  }
  
  interface Meta {
    meta: {
      limit: number;
      page: number;
      total_page: number;
    };
  }
  
  export type ApiDataResponseMeta<T> = ApiDataResponse<T> & Meta;
  
  type ParsedDataResponseMeta<T> = {
    outlet: any;
    data: T;
    meta: {
      limit: number;
      page: number;
      total_page: number;
    };
  }
  
  type ParsedDataResponseWithoutMeta<T> = {
    data: T;
  }
  
  export type QueryDataWithPagination = PaginationData & {
    [key: string | number]: any;
  };
  
  type PaginationData = {
    limit: number;
    page: number;
    max?: number;
  }
  
  type QueryDataWithoutPagination = {
    [key: string | number]: any;
  }
  
  type QueryDataWithPaginationObject = {
    query: { [key: string]: string | number | boolean };
    meta: {
      page: number;
      limit: number;
    };
  };

  export type {
    ApiResponse,
    QueryDataWithPaginationObject,
    QueryDataWithoutPagination,
    ParsedDataResponseMeta,
    ParsedDataResponseWithoutMeta
  }
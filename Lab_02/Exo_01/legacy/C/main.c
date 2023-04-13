#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
 int type;
 float val;
 long timestamp;
} ValueStruct;

typedef struct {
 int type;
 float val[10];
 long timestamp;
} MValueStruct;

typedef struct {
 int type;
 char message[21]; // stringa null terminated lung max 20
} MessageStruct;

typedef struct {
 int type;
 union {
 ValueStruct val;
 MValueStruct mvals;
 MessageStruct messages;
 };
} ExportData;

ValueStruct init_value_struct(float val, long timestamp){

    ValueStruct v;

    v.type=1;
    v.val=val;
    v.timestamp=timestamp;
    return v;

}

MValueStruct init_m_value_struct( long timestamp){

    MValueStruct m_val;
    int i;
    m_val.type = 2;
    for (int i=0 ; i<10 ; i++)
    m_val.val[i] = (float) i;
    m_val.timestamp=timestamp;
    return m_val;

}

MessageStruct init_message_struct(char *message){

    MessageStruct m_struct;
    int i;
    m_struct.type = 3;
    strcpy(m_struct.message, message);
    return m_struct;

}


void export(ExportData *data, int n, FILE *fp) {

    fwrite( data, sizeof(ExportData), n, fp);

}

void print_export_data(FILE *fp){

     int i, j;
     ExportData e_data[100];

     fread(e_data, sizeof(ExportData), 100, fp);

for ( i=0 ; i<100 ; i++)
{

      switch (e_data[i].type){

        case 1:
            printf("type = ValueStruct, val = %f, timestamp = %ld\n", e_data[i].val.val, e_data[i].val.timestamp);
            break;
        case 2:
            printf("type = MValueStruct, val = is vector, timestamp = %ld\n", e_data[i].mvals.timestamp);
            printf("///\n");
            for(j=0 ; j<10 ; j++)
            {
                printf("%f",e_data[i].mvals.val[j]);
            }
            printf("\n///\n");
            break;
        case 3:
            printf("type = MessageStruct, message = %s\n", e_data[i].messages.message);
            break;
        default:
            break;

            }
}


}


int main()
{

    int i, j;
    float v[10];
    char message[21] = "Hello World";
    FILE *fp;
    ExportData data[100];
    int s= sizeof(data[0].type);
    ValueStruct val_struct = init_value_struct(2, 1);
    MValueStruct m_val_struct = init_m_value_struct(1);
    MessageStruct m_struct = init_message_struct(message);
    for ( i = 0, j=1 ; i < 100 ; i++, j++){
            switch (j){

        case 1:
            data[i].type = 1;
            data[i].val = val_struct;
            break;
        case 2:
            data[i].type = 2;
            data[i].mvals = m_val_struct;
            break;
        case 3:
            data[i].type = 3;
            data[i].messages = m_struct;
            j=0;
            break;
        default:
            j = 0;
            break;

            }
    }

    fp = fopen("./myFile.bin","wb");
    export(data, 100, fp);
    fclose(fp);
    fp = fopen("./myFile.bin","rb");
    print_export_data(fp);
    fclose(fp);
    /*
    fclose(fp);
    fp = fopen("./myFile.bin","rb");

    ExportData data_copy[1];
    fread(data_copy, sizeof(ExportData), 1, fp);
    printf("------- %d",data_copy[0].type);
    */
    return 0;
}

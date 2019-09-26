/**
 ********************************************************************************************************
 * @file        log.h
 * @brief   
 ********************************************************************************************************
 */

#ifndef OCKAM_LOG_H_
#define OCKAM_LOG_H_


/*
 ********************************************************************************************************
 * @defgroup    OCKAM_LOG OCKAM_LOG_API
 * @ingroup     OCKAM
 * @brief       OCKAM_LOG_API
 *
 * @addtogroup  OCKAM_LOG
 * @{
 ********************************************************************************************************
 */


/*
 ********************************************************************************************************
 *                                             INCLUDE FILES                                            *
 ********************************************************************************************************
 */

#include <ockam/define.h>


/*
 ********************************************************************************************************
 *                                                DEFINES                                               *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                               CONSTANTS                                              *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                               DATA TYPES                                             *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                          FUNCTION PROTOTYPES                                         *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                            GLOBAL VARIABLES                                          *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                           GLOBAL FUNCTIONS                                           *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                            LOCAL FUNCTIONS                                           *
 ********************************************************************************************************
 */

#ifdef __cplusplus
extern "C" {
#endif

OCKAM_ERR ockam_log_init(void);

OCKAM_ERR ockam_log(void* p_str, uint32_t str_size);


#ifdef __cplusplus
}
#endif

/*
 ********************************************************************************************************
 * @}
 ********************************************************************************************************
 */

#endif
